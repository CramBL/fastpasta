#!/bin/bash

# This script runs the binary fastPASTA and compares the output to the expected output

TXT_RED="\e[31m"
TXT_YELLOW="\e[33m"
TXT_GREEN="\e[32m"
TXT_BLUE="\e[34m"
TXT_BRIGHT_YELLOW="\e[93m"
TXT_BRIGHT_CYAN="\e[96m"
TXT_BRIGHT_MAGENTA="\e[95m"
TXT_BRIGHT_GREEN="\e[92m"
TXT_CLEAR="\e[0m"

# Prefix for each command.
## Run the binary and go to the test-data folder
cmd_prefix="cargo run -- ./test-data/"

# Arrays to store the failed tests
## The index of each array corresponds to the index of the failed test in the tests_array
failed_tests=()
## The matches of each failed test
failed_matches=()
## The output of each failed test
failed_results=()

### Regex patterns ###

## Matches the EOF and Exit Successful messages
## Needs -v2 to show "INFO -" prints
re_eof="((INFO -).*((EOF)|(Exit Successful))*)"
## Matches the RDHs in the `view rdh` command, by going from the `:` in the memory offset to the version, header size, and data format.
re_rdhs_in_rdh_view=": .* (7|6) .* 64 .* (0|2)"

# Tests are structured in an array of arrays
# Each test is an array of 3 elements
tests_array=(
    test_1_0 test_1_1 test_1_2 test_1_3 test_1_4 test_1_5 test_1_6 test_1_7 test_1_8 test_1_9 test_1_10 test_1_11
    test_2_0 test_2_1 test_2_2 test_2_3 test_2_4 test_2_5 test_2_6 test_2_7 test_2_8 test_2_9
    test_3_0 test_3_1 test_3_2 test_3_3 test_3_4 test_3_5 test_3_6 test_3_7 test_3_8
)
# The 3 elements of a test is:
# 0: Command to run
# 1: Regex to match against stdout
# 2: Number of matches expected

### Tests on the `readout.superpage.1.raw` file
## Test 1_0: `check sanity` - Check that the program reached EOF and exits successfully
test_1_0=(
    "readout.superpage.1.raw check sanity -v2"
    "${re_eof}"
    2
)
## Test 1_1: `check sanity` - Check the right data format is detected
test_1_1=(
    "readout.superpage.1.raw check sanity"
    "Data Format.* 2"
    1
)
## Test 1_2: `check sanity its` - Check the right rdh version is detected
test_1_2=(
    "readout.superpage.1.raw check sanity its"
    "rdh version.*7"
    1
)
## Test 1_3: `check all` - Check the right link is detected
test_1_3=(
    "readout.superpage.1.raw check all"
    "links .*2"
    1
)
## Test 1_3: `check all its` - Check the right amount of RDHs is detected
test_1_4=(
    "readout.superpage.1.raw check all"
    "total rdhs.*6"
    1
)
## Test 1_5: `check all its` - Check the right amount of HBFs is detected
test_1_5=(
    "readout.superpage.1.raw check all its"
    "total hbfs.*3"
    1
)
## Test 1_6: `check sanity` - Check the right layers and staves are detected
test_1_6=(
    "readout.superpage.1.raw check sanity"
    "((layers)|(staves)).*((layers)|(staves)).*L1_6"
    1
)
## Test 1_7: `view rdh` - Check the right amount of RDHs is shown
test_1_7=(
    "readout.superpage.1.raw view rdh"
    "$re_rdhs_in_rdh_view"
    6
)
## Test 1_8 `view hbf` - Check the right amount of IHWs is shown
test_1_8=(
    "readout.superpage.1.raw view hbf"
    ": IHW "
    3
)
## Test 1_9 `view hbf` - Check the right amount of TDHs is shown
test_1_9=(
    "readout.superpage.1.raw view hbf"
    ": TDH "
    3
)
## Test 1_10 `view hbf` - Check the right amount of TDTs is shown
test_1_10=(
    "readout.superpage.1.raw view hbf"
    ": TDT "
    3
)
## Test 1_11 `view hbf` - Check the right amount of DDWs is shown
test_1_11=(
    "readout.superpage.1.raw view hbf"
    ": DDW "
    3
)

# Tests on the `10_rdh.raw` file
## Test 2_0: sanity check that the program reached EOF and exits successfully
test_2_0=(
    "10_rdh.raw check sanity -v2"
    "${re_eof}"
    2
)
## Test 2_1: `check sanity` - Check the right RDH version is detected
test_2_1=(
    "10_rdh.raw check sanity"
    "RDH.*Version.*7"
    1
)
## Test 2_2: `check sanity` - Check the right number of RDHs is detected
test_2_2=(
    "10_rdh.raw check sanity"
    "Total.*RDHs.*10"
    1
)
## Test 2_3: `check sanity` - Check the right number of HBFs is detected
test_2_3=(
    "10_rdh.raw check sanity"
    "Total.*hbfs.*5"
    1
)
## Test 2_4: `view rdh` - Check the right number of RDHs is shown
test_2_4=(
    "10_rdh.raw view rdh"
    "$re_rdhs_in_rdh_view"
    10
)
## Test 2_5: `view hbf` - Check the right number of RDHs is shown
test_2_5=(
    "10_rdh.raw view hbf"
    ": RDH"
    10
)
## Test 2_6: `view hbf` - Check the right number of IHWs is shown
test_2_6=(
    "10_rdh.raw view hbf"
    ": IHW"
    5
)
## Test 2_7: `view hbf` - Check the right number of TDHs is shown
test_2_7=(
    "10_rdh.raw view hbf"
    ": TDH"
    5
)
## Test 2_8: `view hbf` - Check the right number of TDTs is shown
test_2_8=(
    "10_rdh.raw view hbf"
    ": TDT"
    5
)
## Test 2_9: `view hbf` - Check the right number of DDWs is shown
test_2_9=(
    "10_rdh.raw view hbf"
    ": DDW"
    5
)

# Tests on the `err_not_hbf.raw` file
## Test 3_0: sanity check that the file is parsed successfully
test_3_0=(
    "err_not_hbf.raw check sanity -v2"
    "${re_eof}"
    2
)
## Test 3_1: `check all` - Check the right number of errors are detected
test_3_1=(
    "err_not_hbf.raw check all"
    "(error - 0xa0.*pages)|(Total Errors.*[0-9])"
    2
)
## Test 3_2: `check sanity` - Check the right number of errors are detected
test_3_2=(
    "err_not_hbf.raw check sanity"
    "error - "
    0
)
## Test 3_3: `view rdh` - Check the right number of RDHs is shown
test_3_3=(
    "err_not_hbf.raw view rdh"
    "$re_rdhs_in_rdh_view"
    2
)
## Test 3_4: `view hbf` - Check the right number of RDHs is shown
test_3_4=(
    "err_not_hbf.raw view hbf"
    ": RDH "
    2
)
## Test 3_5: `view hbf` - Check the right number of IHWs is shown
test_3_5=(
    "err_not_hbf.raw view hbf"
    ": IHW "
    2
)
## Test 3_6: `view hbf` - Check the right number of TDHs is shown
test_3_6=(
    "err_not_hbf.raw view hbf"
    ": TDH "
    2
)
## Test 3_7: `view hbf` - Check the right number of TDTs is shown
test_3_7=(
    "err_not_hbf.raw view hbf"
    ": TDT "
    2
)
## Test 3_8: `view hbf` - Check the right number of DDWs is shown
test_3_8=(
    "err_not_hbf.raw view hbf"
    ": DDW "
    0 # There are no DDWs in this file as it is an erroneous file
)

echo -e "Running ${TXT_BRIGHT_YELLOW}${#tests_array[@]}${TXT_CLEAR} regression tests"

for test in "${tests_array[@]}"; do
    declare -n current_test=$test
    test_case=${current_test[0]}
    pattern=${current_test[1]}
    cond=${current_test[2]}
    echo -e "running ${TXT_BRIGHT_MAGENTA}${test}${TXT_CLEAR}: ${TXT_BRIGHT_YELLOW}${test_case}${TXT_CLEAR}"
    echo -e "Condition is: ${TXT_BLUE}[number of matches] == ${cond}${TXT_CLEAR}, for pattern: ${TXT_BRIGHT_CYAN}${pattern}${TXT_CLEAR}"
    # Run the test, redirecting stderr to stdout, and skipping the first 2 lines (which are the "Finished dev..., Running..." lines)
    test_out=$(eval ${cmd_prefix}${test_case} 2>&1 | tail -n +3 )
    matches=$(echo "${test_out}" | egrep -i -c "${pattern}")
    #echo -e "matches:${matches}";
    if (( "${matches}" == "${cond}" ));
    then
        echo -e "${TXT_GREEN}Test passed${TXT_CLEAR}"
    else
        echo -e "${TXT_RED}Test failed${TXT_CLEAR}"
        failed_tests+=("${test}")
        failed_matches+=("${matches}")
        failed_output+=("${test_out}")
    fi;
done

echo
if  [[ "${#failed_tests[@]}" == 0 ]];
then
    echo -e "${TXT_BRIGHT_GREEN}ALL TESTS PASSED! :)${TXT_CLEAR}"
    exit 0
else
    echo -e "${TXT_RED}${#failed_tests[@]} Failed test(s):${TXT_CLEAR}"
    for (( i = 0; i < ${#failed_tests[@]}; i++ )); do
        declare -n failed_test=${failed_tests[i]}
        echo -e "${TXT_RED}${failed_tests[i]}${TXT_CLEAR}: ${failed_test[0]}"
        echo -e "${TXT_BRIGHT_CYAN}Pattern: ${TXT_CLEAR}${failed_test[1]}"
        echo -e "${TXT_BRIGHT_YELLOW}Expected:${TXT_CLEAR} ${failed_test[2]} ${TXT_BRIGHT_YELLOW}Got:${TXT_CLEAR} ${failed_matches[i]}"
        echo -e "${TXT_BRIGHT_MAGENTA}Test output: ${TXT_CLEAR}"
        echo -e "${failed_output[i]}"
    done
    exit 1
fi
