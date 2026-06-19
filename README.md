# Offsync

Offsync is a command-line tool to synchronize a local directory with remote ones without having to
be connected at the same time to the internet.

## Background

Our main purpose is to practice and learn about the Rust programming language and its ecosystem as
part of the Rust community in
[Girona](https://www.openstreetmap.org/search?lat=41.97943&lon=2.81632#map=11/42.0502/2.1949)).

After that we didn't find an active experienced Rust developers in Girona to grow the Rust community
in Girona, we decided to start a series of meetups to attract more people to Rust. These meetups are
mostly focused to people that are new to Rust and want to learn more about it.

We had the first meetup the [4th of December of 2024](https://lu.ma/pypwr0m7) and the attendees
decided that they want to learn Rust by building a project together.

The second meetup was the [8th of January of 2025](https://lu.ma/ckf2s00f) and we chose this
project.

After that we didn't make much progress until June of 2026, which in our [onsite
meetup](https://luma.com/3bcnx1jb), the nowadays frequent assistants we decided to work in the
project during the onsite meetups, while keeping going with the
[exercises](https://github.com/rust-girona/rust-course-fei-solutions) during the online ones.

We welcome anyone that wants to join us in this project asynchronously or in the meetups and with
any level of experience in Rust. People new to Rust will learn, people experienced in Rust will
help others, and everybody will have fun and boost the Girona Rust community.

## Main use case

Pep stores historical files (e.g. Holidays photos, finished projects, etc.) in an external disk.

Pep is worried that his disk could fail. He wants to have redundancy in case of failure, but he's
worried that if he stores all the disks in the same location, the redundancy won't be useful in case
of a disaster (e.g. Fire, flood, etc.). He doesn't need to access the files immediately in case of a
disaster.

Pep could use a cloud storage service, but he doesn't want to add that recurrent cost to his budget.
He doesn't want to have a device connected 24/7 for this purpose to avoid extra electricity costs
and increase his environmental footprint.

Pep doesn't want to have to go to a physical location to sync the disks.

Pep wants to rely on his friends and family to store the disks in their homes. He wants to be able
to sync the data changes when he stores them in the disk that he has at home, but he doesn't want
to limit those moments with his friends' availability.

## How it works?

Based on the main use case.

Pep stores data in the disk that he has at home. He create copies of them and distribute one copy to
a different friend or family member.

Pep stores new files and modified files, and delete files in the disk that he has at home. He
executes `offsync`. `offsync` detects the changes and stores them in a configured cloud storage
provider. Because the changes shouldn't take a lot of storage, he uses a free tier of a cloud
storage provider.

Pep's friends and family execute `offsync` in their homes when they find a convenient time.
`offsync` finds the changes in the cloud storage provider and sync them in their disks. When
`offsync` deletes the files in the cloud storage provider when it has synced them and there are not
more locations out of sync.

Because `offsync` source/destination is a directory, the data can be in a external drive or in the
same computer. His friends and family could potentially use the disk to the same as him and they
could use the disk to store their own data backups, so they could share the cost of the disks and
all of them would have an incentive to execute `offsync` frequently to keep the changed data in the
cloud storage provider for shorter periods.

See
[docs/specification-functional.md](https://github.com/rust-girona/offsync/blob/main/docs/specification-functional.md).

## Status

The project is in the early stages of development.

We plan to offer the minimum functionality that satisfies [the main use case](#main-use-case) when
we reach v1.

The minimum functionality is:
- Synchronize to only one destination (i.e. backup).
- Change can only be made in the source, not in the backup.
- Implement the support for only one cloud as s sync store, a part of a mock one mapped to a local
  path for testing purposes.
- Data is stored in plain, no encryption. We assume that all the locations are trusted. We don't
  even contemplate other security issues for this version.

A more detailed specification of this functionality is in
[docs/specification-functional.md](https://github.com/rust-girona/offsync/blob/main/docs/specification-functional.md).
