SELECT 
    event_id, 
    payload:device::string AS device_type 
FROM user_events 
QUALIFY ROW_NUMBER() OVER (PARTITION BY event_id ORDER BY event_id) = 1;
