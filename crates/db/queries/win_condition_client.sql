--: WinCondition()
 
--! get_win_conditions : WinCondition
SELECT id, code, created_at
FROM tagteam.win_condition;

--! get_win_condition_by_id : WinCondition
SELECT id, code, created_at
FROM tagteam.win_condition
WHERE id = :id;