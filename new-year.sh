#!/bin/sh

year="$1"

if [ -z "$year" ];
then
	echo "Usage: $0 <year>"
	exit 1
fi

package_name="d${year}"

pushd "$(dirname $0)"

if [ -d "$package_name" ];
then
	echo "Directory ${package_name} already exists... not doing anything!"
	exit 1
fi

echo ">>> Creating cargo package, \"${package_name}\""

cargo init --lib "${package_name}"

if [ $? -ne 0 ];
then
	echo "Something seems to have gone wrong... not going to touch the directory."
	exit 1
fi

echo ">>> /!\\ Please add the cargo package to the workspace members. /!\\"
read -p ">>>     When done, type \"done\": " input

if [ "$input" != "done" ];
then
	echo "You did not type the word \"done\" so I am not doing anything."
	exit 1
fi

echo ">>> Setting up cargo package ${package_name}"

pushd "${package_name}"

echo ">>> Adding common dependencies"

cargo add --path "../daocutil"

for package in anyhow thiserror itertools nu-ansi-term regex;
do
	cargo add "$package"
done

echo ">>> Making extra directories"

mkdir -pv inputs src/examples

echo ">>> Creating .keep files"

touch inputs/.keep src/examples/.keep

echo ">>> Setting .gitattributes for inputs/ folder"

echo "inputs/*	linguist-generated=true" > .gitattributes

echo ">>> Adding stub for src/lib.rs"

touch src/lib.rs

echo ">>> Creating day parts"

for daynum in $(seq 1 25);
do
	day_identifier=$(printf "day%02d" $daynum)

	echo "pub type Intermediate = ();
pub type Output = u32;

/// # Errors
pub fn parse(_data: &str) -> anyhow::Result<Intermediate> {
	Ok(())
}

#[must_use]
pub fn part_one(_intermediate: &Intermediate) -> Option<Output> {
	None
}

#[must_use]
pub fn part_two(_intermediate: &Intermediate) -> Option<Output> {
	None
}" > "src/${day_identifier}.rs"
	echo "pub mod ${day_identifier};" >> src/lib.rs
done

echo "daocutil::generate_main!(${package_name});" > src/main.rs
