#! /bin/sh
#
# build.sh
#
#set -e

mkdir -p /tmp/ractl/ractl-$VERSION/usr/bin SOURCES
cp ../target/release/ractl /tmp/ractl/ractl-$VERSION/usr/bin
tar czf ./SOURCES/ractl-$VERSION.tar.gz -C /tmp/ractl ractl-$VERSION

RPM_BUILD_SPECS=$(rpmbuild --nobuild --eval '%{_specdir}')
RPM_BUILD_SOURCES=$(rpmbuild --nobuild --eval '%{_sourcedir}')
RPM_BUILD_RPMS=$(rpmbuild --nobuild --eval '%{_rpmdir}')
RPM_BUILD_SRPMS=$(rpmbuild --nobuild --eval '%{_srcrpmdir}')


find ./SPECS -mindepth 1 -exec cp --verbose -R {} ${RPM_BUILD_SPECS}/. \;
find ./SOURCES -mindepth 1 -exec cp --verbose -R {} ${RPM_BUILD_SOURCES}/. \;

for spec_file in ${RPM_BUILD_SPECS}/*.spec; do
    spectool --sourcedir --get-file $spec_file
    rpmbuild -ba $spec_file
done


echo ================================================

ls -l ${RPM_BUILD_SRPMS}/*
ls -l ${RPM_BUILD_RPMS}/*/*

rm -rf /tmp/ractl

