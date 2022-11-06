SELECT  *,
        title,
        platforms,
        released
FROM video_games AS vg
LEFT JOIN publishers AS p
ON vg.publisher_id = p.id;
;