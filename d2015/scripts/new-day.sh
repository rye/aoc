#!/bin/sh

workdir="$(dirname $0)/.."
year="2015"

daynumber="$1"
daynumber2d="$(printf '%02d' $daynumber)"

if [ -d "${workdir}/src/day${daynumber2d}" ];
then
	>&2 echo "Directory src/day${daynumber2d} already exists. Not creating again."
else
	mkdir -pv "${workdir}/src/day${daynumber2d}"
fi

if [ -f "${workdir}/src/day${daynumber2d}/mod.rs" ];
then
	>&2 echo "Module src/day${daynumber2d}/mod.rs already exists. Not creating again."
else
	cat > "${workdir}/src/day${daynumber2d}/mod.rs" << EOM
type Intermediate = ();

pub fn parse(_input: &str) -> Intermediate {}

type Solution = ();

pub fn part_one(_intermediate: &Intermediate) -> Option<Solution> {
	None
}

pub fn part_two(_intermediate: &Intermediate) -> Option<Solution> {
	None
}
EOM
fi

if [ -f "${workdir}/src/bin/day${daynumber2d}.rs" ];
then
	>&2 echo "Binary src/bin/day${daynumber2d}.rs already exists. Not creating again."
else
	cat > "${workdir}/src/bin/day${daynumber2d}.rs" << EOM
d${year}::day_solver_from!(d${year}::day${daynumber2d});
EOM
fi
