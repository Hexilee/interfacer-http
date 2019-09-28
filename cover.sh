zip -0 ccov.zip $(find . \( -name "interfacer_http*.gc*" \) -print) &&
grcov ccov.zip -s . -t coveralls \
--token "$COVERALL_TOKEN" \
--commit-sha "$TRAVIS_COMMIT" \
--service-number "$TRAVIS_BUILD_NUMBER" \
--service-job-number "$TRAVIS_JOB_NUMBER" \
--service-name 'interfacer-http' \
--llvm \
--ignore-not-existing \
--ignore-dir "target/*" "/*" \
-o output.json &&
curl -F json_file=@output.json "https://coveralls.io/api/v1/jobs" -vvv