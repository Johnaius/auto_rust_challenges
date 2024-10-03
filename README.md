# Automated Rust Challenges Repo
I wanted to create a repo to practice coding challenges daily.  i wanted to automate daily creation of rust project as well as automate running each project with a script.  I would like to add fucntionality to the run script to automatically push upon a successful run, but would need to write tests each day.

## Create a New Project for Each Day

To create a new project for each day, use the following command while in the root dir:

```sh
./create_challenge.sh <DAY#>
```

## Run the Challenge

To run the challenge for a specific day, use the following command while in root dir:

```sh
./run_challenge.sh <DAY#>
```