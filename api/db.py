from sqlalchemy.orm import DeclarativeBase
from sqlalchemy import Column, Integer, String, Float
from sqlalchemy.ext.asyncio import AsyncSession, create_async_engine, async_sessionmaker

DATABASE_URL = "sqlite+aiosqlite:///./logs.db"

# ✅ Use ONLY the async engine
async_engine = create_async_engine(DATABASE_URL, echo=True)
AsyncSessionLocal = async_sessionmaker(async_engine, expire_on_commit=False)

class Base(DeclarativeBase):
    pass

class Logs(Base):
    __tablename__ = "Logs"
    id = Column(Integer, primary_key=True, index=True)
    log = Column(String, index=True)

async def create_db_and_tables():
    async with async_engine.begin() as conn:
        # This helper runs the sync 'create_all' in an async context
        await conn.run_sync(Base.metadata.create_all)

async def get_async_session():
    async with AsyncSessionLocal() as session:
        yield session