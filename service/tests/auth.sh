
curl 'http://localhost:8080/api/v1/auth' \
  -H 'content-type: application/json' \
  --data-raw '{"email":"Felix@Dom.ke","password":"pwd"}' -c cookies

cat cookies

curl 'http://localhost:8080/api/v1/auth' -b cookies
