CREATE PROCEDURE solution()
BEGIN
	SELECT id, email_title, (
		CASE
			WHEN size / POWER(2, 20) >= 1 THEN CONCAT(FLOOR(size / POWER(2, 20)), ' Mb')
			ELSE CONCAT(FLOOR(size / POWER(2, 10)), ' Kb')
		END
	) AS short_size
	FROM emails
	ORDER BY size DESC;
END
