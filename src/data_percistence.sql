-- //sql to calculate the success rate of deliveries for each driver in the last month, and filter out drivers with a success rate below 90% or less than 5 completed deliveries.

SELECE 
    driver_id,
    COUNT(*) AS total_deliveries,
    (COUNT(CASE WHEN delivery_status = 'COMPLETED' THEN 1 END)* 100.0 / COUNT(*)) AS success_rate
FROM 
    deliveries
WHERE 
    attempt_timestamp >= '2024-09-01' AND attempt_timestamp <= '2024-10-01'
GROUP BY 
    driver_id
HAVING 
    COUNT(CASE WHEN delivery_status = 'COMPLETED' THEN 1 END)>= 5
    AND (COUNT(CASE WHEN delivery_status = 'COMPLETED' THEN 1 END)* 100.0 / COUNT(*)) >= 90
ORDER BY 
    success_rate DESC;