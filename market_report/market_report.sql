CREATE PROCEDURE solution()
BEGIN
	SELECT *
	FROM (
		SELECT country, COUNT(country) AS competitors
		FROM foreignCompetitors
		GROUP BY country
		ORDER BY country
	) AS orderedCompetitors
	UNION
	SELECT 'Total:', COUNT(*)
	FROM foreignCompetitors;
END