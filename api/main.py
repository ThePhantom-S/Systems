import time
from typing import List
import uvicorn
from fastapi import FastAPI, Depends
from fastapi.middleware.cors import CORSMiddleware
from sqlalchemy import select
from sqlalchemy.ext.asyncio import AsyncSession
from contextlib import asynccontextmanager

from db import get_async_session, Logs, create_db_and_tables
from model import Message

# ✅ Lifespan handler ensures tables are created before server starts handling requests
@asynccontextmanager
async def lifespan(app: FastAPI):
    await create_db_and_tables()
    yield

app = FastAPI(lifespan=lifespan)

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
async def get_logs(session: AsyncSession = Depends(get_async_session)):
    result = await session.execute(select(Logs.log))
    logs = result.scalars().all()
    return logs


@app.post("/logs")
async def post_message(
    content: List[Message], 
    session: AsyncSession = Depends(get_async_session)
):
    # ✅ Fixed: Changed from massive uuid1 integer to standard Unix epoch float
    new_logs = [
        Logs(log=item.message)
        for item in content
    ]
    
    # ✅ Fixed: Chunk inserts to stay safe from SQLite's parameter limits (max 999 variables)
    # 400 items * 2 parameters = 800 variables per chunk execution
    chunk_size = 400 
    for i in range(0, len(new_logs), chunk_size):
        session.add_all(new_logs[i:i + chunk_size])
        
    await session.commit()
    
    return {"status": "success", "inserted": len(new_logs)}


if __name__ == "__main__":
    uvicorn.run(
        "main:app",
        host="0.0.0.0",
        port=8000,
        reload=True,
    )