# shellcheck source=./util.sh
_dir="$( dirname "$0" )"
[ -f "${_dir}/util.sh" ] || bash "${_dir}/download-util.sh" || exit 1
source "${_dir}/util.sh"
unset _dir

set -e

SCRIPT_NAME="$0"
CRATE_NAME_PREFIX="aoc2019-"

function get_day_crate_name_from_args {
    local day="$1"
    local crate_name="${CRATE_NAME_PREFIX}${day}"

    [ -z "$day" ] && err "\
First argument must be the day.
Example usage:
    $( colored "$COLOR_CODE" "$SCRIPT_NAME day-01" )"

    echo "$crate_name"
}
