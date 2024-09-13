CREATE OR REPLACE VIEW penalty_view AS 

SELECT 

p.id,
p.employee_id,
p.amount,
p.description,
p.until,
e.name AS employee_name,
p.created_at,
p.updated_at,
p.deleted_at

FROM `penalties` AS p 

JOIN (`employees` AS e)

ON (e.id = p.employee_id)