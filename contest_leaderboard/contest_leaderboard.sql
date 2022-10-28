CREATE PROCEDURE solution()
BEGIN
	SELECT name
	FROM leaderboard
	WHERE score < (
		SELECT MIN(score)
		FROM (
			SELECT score
			FROM leaderboard
			ORDER BY score DESC
			LIMIT 3
		) AS top3Board
	)
	ORDER BY score DESC
	LIMIT 5;
END