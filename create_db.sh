docker run --name memosdb \
	-e POSTGRES_PASSWORD=devpassword \
	-e POSTGRES_USER=devuser \
	-v /home/markusamuli/Developer/memos/.db_data:/var/lib/postgresql/data \
	-p 5432:5432 \
	-d \
	postgres:14
