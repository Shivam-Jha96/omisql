SELECT title, first_name, last_name, last_update
FROM film
JOIN film_actor ON film.film_id = film_actor.film_id
JOIN actor ON film_actor.actor_id = actor.actor_id;

-- Phase 3 Tests
SELECT title FROM film WHERE 0 = 1;
SELECT title FROM film WHERE 1 = x / 0;
