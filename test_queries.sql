SELECT title, first_name, last_name, last_update
FROM film
JOIN film_actor ON film.film_id = film_actor.film_id
JOIN actor ON film_actor.actor_id = actor.actor_id;
