dev-init:
    k3d cluster delete k3d-nails
    k3d cluster create k3d-nails --agents 1 -p "30000-30001:30000-30001@agent:0"
    just get-config


dev-setup:
    stack init
    stack install --manifest stack.dev.yaml
    
    echo "Waiting for the database to be ready..."
    sleep 30
    echo "Database should be ready now."

    just generate-openapi

    dbmate up
    just generate-db-client

# Retrieve the cluster kube config - so kubectl and k9s work.
get-config:
    k3d kubeconfig write k3d-nails --kubeconfig-merge-default
    sed -i "s/127\.0\.0\.1/host.docker.internal/g; s/0\.0\.0\.0/host.docker.internal/g" "$HOME/.kube/config"
    # Disable TLS verification for local dev
    sed -i '/certificate-authority-data/d' "$HOME/.kube/config"
    sed -i '/cluster:/a \ \ \ \ insecure-skip-tls-verify: true' "$HOME/.kube/config"
    echo "✅ kubeconfig updated and TLS verification disabled"

install-openapi-generator:
    cd openapi && npm i
    echo "✅ OpenAPI Generator CLI installed"

generate-openapi:
    echo "Generating OpenAPI client..."
    cd openapi && \
    npx openapi-generator-cli generate -g rust-axum -o ../crates/openapi -i api.yaml
    echo "✅ OpenAPI client generated"

generate-db-client:
    echo "Generating database client..."
    clorinde live -q ./crates/db/queries/ -d ./crates/clorinde/
    echo "✅ Database client generated"