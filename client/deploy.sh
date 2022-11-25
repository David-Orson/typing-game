VERSION=`git describe --always`

echo $VERSION

docker build -t orsondc/type-hero-vue . && \
docker push orsondc/type-hero-vue