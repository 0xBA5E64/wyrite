if docker inspect wyrite-db >& /dev/null; then
    echo Container exists
    if docker inspect wyrite-db | grep '"Status": "running"' >& /dev/null; then
        echo ...and running. Exiting.
    else
        echo ...but not running. Starting container...
        docker start wyrite-db
        echo ...container started. Exiting.
    fi
else
    echo Container does not exist. Creating...

    docker run --name wyrite-db -e POSTGRES_USER=wyrite -e POSTGRES_PASSWORD=notsosecret -e PGDATA=/var/lib/postgresql/data/pgdata -v ./.db-data/:/var/lib/postgresql/data -p 5000:5432 -d postgres
    sqlx migrate run
    #cat sample-data.sql | docker exec -i wyrite-db psql -Uwyrite-db
    echo Start the server and run ./insert-some-posts.sh to populate the database with some sample data
fi

