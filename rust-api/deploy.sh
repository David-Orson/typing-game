VERSION=`git describe --always`

echo $VERSION

docker build -t orsondc/type-hero-api . && \
docker push orsondc/type-hero-api
