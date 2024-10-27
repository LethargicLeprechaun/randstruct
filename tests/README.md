# Randstruct Tests
---
## Testing Overview
There are unit tests in the crate's lib.rs which test a lot of the trivial functionality. The entry point for running these tests is the `run_tests.sh` shell script which will vary the `randstruct_seed` per build. It then uses the test cases in the truth dataset `test_cases.json`, this does incur rebuilding the crate for every test case.

## Adding a New Test Case
To generate a new test case, you need to compile `helpers/generate_test_case.c` with the GCC randomize_layout plugin and with a new `randstruct_seed`. I have included `helpers/test.c` and `helpers/generate_json.py` to aid in the generation of test cases.

For example (assuming you have built the randstruct plugin with a new seed):
```bash
gcc test.c -fplugin=`pwd`/randomize_layout_plugin.so -o test
./test > test_case.csv
python3 generate_json.pyt --csv test_case.csv <seed value> > test_case.json
```

This will produce a json file called `test_case.json` which can be included in `test_cases.json`. You will also need to add the seed's value to the array in `run_tests.sh` to make sure it is run as part of the wider tests.