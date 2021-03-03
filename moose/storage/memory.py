from moose.storage.base import DataStore


class MemoryDataStore(DataStore):
    def __init__(self, initial_store={}):
        self.store = initial_store

    async def load(self, session_id, key, query):
        return self.store[key]

    async def save(self, session_id, key, value):
        self.store[key] = value
