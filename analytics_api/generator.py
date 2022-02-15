#!/usr/bin/env python3
"""
# Copyright 2022 by Michael Lichtenecker.
# All rights reserved.

Sample data generator that is used in development to generate sample data to test the analytic api and the analyse endpoints of this service. 
"""

from dataclasses import dataclass
import random


# env data
SERVER = "localhost:20819"
APPID = ""
TOKEN = ""
AMOUNT = 100


# data collections
device_types = ["iPhone11", "GalaxyS20", "iPhone13", "GalaxyS22", "iPhone12", "iPad Pro", "a20", "Nord2"]
countrys = ["DE", "EN"]
os = ["ios", "android"]


@dataclass(init=True)
class SampleData:
    os: str
    device_size: str
    new_user: bool
    country: str
    session_length: int
    device_type: str
    version: str

    def generate(self):
        self.device_type = get_random_value(device_types)
        self.os = get_random_value(os)
        self.country = get_random_value(countrys)
        self.device_size = f"{random.randint(300, 600)}x{random.randint(600, 900)}"


    def send_data(self):
        pass

def get_random_value(data: list) -> str:
    return data[random.randrange(len(data) - 1]

def __name__ == "__main__":
    pass

