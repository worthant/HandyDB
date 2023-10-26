#!/bin/bash

deploy_war_to_helios() {
    echo "Deploying to Helios"

    # Remove existing deployment
    ssh -p 2222 s368090@se.ifmo.ru "rm -rf wildfly/wildfly-21.0.0.Final/standalone/deployments/MVC-GeoValidator.war"
    # add new deployment
    scp -P 2222 ./target/MVC-GeoValidator.war s368090@se.ifmo.ru:wildfly/wildfly-21.0.0.Final/standalone/deployments
}

deploy_client_and_server_to_helios() {
    echo "Deploying to Helios"

    # Remove existing directory
    ssh -p 2222 s368090@se.ifmo.ru "rm -rf ~/public_html/lab1/"

    # Create new directory
    ssh -p 2222 s368090@se.ifmo.ru "mkdir -p ~/public_html/lab1"

    # Copy client files
    scp -P 2222 -r ./client/* s368090@se.ifmo.ru:~/public_html/lab1/

    # Copy server files
    scp -P 2222 -r ./server/* s368090@se.ifmo.ru:~/public_html/lab1/
}

deploy_to_github_pages() {
    echo "Deploying to GitHub"
    yarn build
    cd dist
    git init
    git add -A
    git commit -m "New Deployment"
    git push -f git@github.com:repo/frontend.git master:gh-pages
    cd -
}

while [ -n "$1" ]; do
    case $1 in
    --war_helios) deploy_war_to_helios ;;
    --helios) deploy_client_and_server_to_helios ;;
    --github) deploy_to_github_pages ;;
    *) echo "Option $1 not recognized" ;;
    esac
    shift
done
