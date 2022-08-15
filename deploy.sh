set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly TARGET_HOST=<user>@<host>
readonly TARGET_PATH=/home/<user>/piwrmon
readonly TARGET_ARCH=aarch64-unknown-linux-gnu
readonly SOURCE_PATH=./target/${TARGET_ARCH}/release/piwrmon

cargo build --release --target=${TARGET_ARCH}
rsync ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}
