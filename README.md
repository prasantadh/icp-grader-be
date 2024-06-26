# icp-grader-be
Backend for grading papers online.

## Instructions to run
The program expects some environment variables. The environment variables are
provided using a `.env` file on the root folder of the project.

```env
MONGO_INITDB_ROOT_USERNAME="username"
MONGO_INITDB_ROOT_PASSWORD="password"
MONGO_CONN_URI="mongodb://username:password@localhost:27017/"
DB_NAME="icp_grader"

GOOGLE_OAUTH_CLIENT="client"
GOOGLE_OAUTH_SECRET="<ask-me-for-the-secret>"
GOOGLE_OAUTH_RETURN="http://127.0.0.1:8080/auth_return"
JWT_SIGNING_SECRET="secret-goes-here"
ADMIN_EMAIL="me_be_admin@admining@icp.np"
```

Run the program with
```bash
docker compose up --build
```
The service will be listening on `http://localhost:80/`
