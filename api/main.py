import uvicorn
from fastapi import FastAPI
from sqlalchemy import select
from model import Message
from typing import List
from sqlalchemy.ext.asyncio import AsyncSession
from fastapi.middleware.cors import CORSMiddleware
from db import create_db_and_tables, get_async_session, Logs
import uuid

app = FastAPI()

origins = [
    "http://localhost:5173",
    "http://127.0.0.1:5173",
]

app.add_middleware(
    CORSMiddleware,
    allow_origins=origins,
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


@app.get("/logs")
def get_logs():
    return logs

@app.post("/logs")
def post_message(content: List[Message]):
    for log in content:
        logs.append({
            "id": str(uuid.uuid4()),
            "message": log.message
        })
    return {"status": "success", "data": content}

if __name__ == "__main__":
    uvicorn.run(
        "main:app",
        host="0.0.0.0",
        port=8000,
        reload=True,
    )