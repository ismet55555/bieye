#!/bin/bash

VERSION="0.2.1"
PPA="ppa:ismet55555/ppa"
# DISTS=("lunar" "jammy" "focal" "bionic" "disco" "xenial")
DISTS=("focal")

START_TIME=$(date +%s)

for DIST in "${DISTS[@]}"; do
	echo "***************************************************"
	echo "*             Distribution: $DIST"
	echo "***************************************************"
	echo "----> Changing version in debian/changelog to $VERSION ..."
	sed -i "1s/(.*;/($VERSION) $DIST;/" debian/changelog

	echo "----> Building source package ..."
	debuild -S

	PACKAGE_SOURCE="bieye_${VERSION}_source.changes"
	echo "----> Uploading source package to PPA: $PPA for package source $PACKAGE_SOURCE ..."
	dput "$PPA" ../"$PACKAGE_SOURCE"

	echo "----> Cleaning up ..."
	rm ../bieye_"$VERSION"*
done

END_TIME=$(date +%s)
TIME_TAKEN=$((END_TIME - START_TIME))
echo "Total time taken: $TIME_TAKEN seconds"
echo "DONE"
