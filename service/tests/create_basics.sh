curl 'http://localhost:8080/api/v1/event' \
  -H 'content-type: application/json' \
  --data-raw '{"name":"new"}' > 0_create_event
  
id=`jq -r ".id" < 0_create_event`

echo "new event ID $id"

curl "http://localhost:8080/api/v1/event/$id" \
  -X 'PUT' \
  -H 'content-type: application/json' \
  --data-raw "{\"id\":\"$id\",\"name\":\"Landes-Schul-Schach-Meisterschaft 2025\",\"public\":true,\"begin\":\"2025-03-01\"}" > 1_create_event

curl "http://localhost:8080/api/v1/event/$id/groups" \
  -H 'content-type: application/json' \
  --data-raw '{"name":"WK I"}'

curl "http://localhost:8080/api/v1/event/$id/groups" \
  -H 'content-type: application/json' \
  --data-raw '{"name":"WK II"}'

curl "http://localhost:8080/api/v1/event/$id/groups" \
  -H 'content-type: application/json' \
  --data-raw '{"name":"WK III"}'

curl "http://localhost:8080/api/v1/event/$id/groups" \
  -H 'content-type: application/json' \
  --data-raw '{"name":"WK IV"}'

curl "http://localhost:8080/api/v1/event/$id/groups" \
  -H 'content-type: application/json' \
  --data-raw '{"name":"WK G"}'
