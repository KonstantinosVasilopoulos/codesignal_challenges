CREATE PROCEDURE solution()
BEGIN
	SELECT id, name, surname
	FROM Suspect
	WHERE height <= 170
		AND UPPER(name) LIKE 'B%'
		AND LOWER(surname) LIKE 'gre_n'
	ORDER BY id;
END
