docker build --no-cache -t reg .

docker run -d -p 5173:8080 -v ./service/test.db:/app/db.sqlite3:rw reg

or

VERSION=`date +%Y%m%d`
docker tag reg ghcr.io/tmbinc/reg:$VERSION
docker push ghcr.io/tmbinc/reg:$VERSION
