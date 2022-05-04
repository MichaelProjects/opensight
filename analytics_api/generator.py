#!/usr/bin/env python3
"""
# Copyright 2022 by Michael Lichtenecker.
# All rights reserved.

Sample data generator that is used in development to generate sample data to test the analytic api and the analyse endpoints of this service.

Use cases:
- demo-console.opensight.io Data Generator
- Development Data Generator (to test the analytic endpoints) 
"""

from dataclasses import dataclass
from datetime import *
import requests
import random
import os
import logging
import json
import time


# env data
SERVER = ""
APPID = ""
TOKEN = ""
SLEEP = 2 # the time that waits between requests
AMOUNT = 1000

# todo implement timeframe 

# data collections
device_types = ["iPhone11", "GalaxyS20", "iPhone13", "GalaxyS22", "iPhone12", "iPad Pro", "a20", "Nord2"]
countrys = ["DE", "EN"]
os = ["ios", "android"]
app_versions = ["1.0.0", "1.1.0", "1.2.0"]


class SampleData:
    create_time: datetime
    os: str
    device_size: str
    new_user: bool
    country: str
    device_type: str
    version: str

    session_id: str
    failed: False

    def __dict__(self):
        return {
            "creation_time": str(self.create_time).replace(" ", "T"),
            "os": self.os,
            "device_size": self.device_size,
            "country": self.country,
            "new_user": self.new_user,
            "device_type": self.device_type,
            "version": self.version,
        }

    def generate(self):
        self.create_time = datetime.now().utcnow()
        self.device_type = get_random_value(device_types)
        self.os = get_random_value(os)
        self.new_user = True if random.randint(0, 1) == 0 else False
        self.country = get_random_value(countrys)
        self.device_size = f"{random.randint(300, 600)}x{random.randint(600, 900)}"
        self.version = get_random_value(app_versions)

    def update_session_length(self):
        first_session_today = True if random.randint(0, 10) >= 3 else False
        uri = f"{SERVER}/analytic/v1/{APPID}/session"
        headers = {"Content-Type": "application/json", "Authorization": f"Bearer {TOKEN}"}
        data = {"session_id": self.session_id, "length": random.randint(5, 1000), "is_first_today": first_session_today}
        response = requests.patch(uri, data=json.dumps(data), headers=headers)
            

    def send_data(self):
        uri = f"{SERVER}/analytic/v1/{APPID}/session"
        headers = {"Content-Type": "application/json", "Authorization": f"Bearer {TOKEN}"}
        response = requests.post(uri, headers=headers, data=json.dumps(self.__dict__()))
        if response.status_code == 200:
            result = response.json()
            self.session_id = result["session_id"]
        else:
            self.failed = True
            logging.error(f"Error sending data: {response.status_code}")


def get_random_value(data: list) -> str:
    return data[random.randrange(len(data) - 1)]


def read_conf ():
    with open("app_conf.json", "r") as f:
        config = json.load(f)

    return config["server"], config["app_id"], config["token"]


if __name__ == "__main__":
    SERVER, APPID, TOKEN = read_conf()
    while True:
        for i in range(AMOUNT):
            sample_data = SampleData()
            sample_data.generate()
            sample_data.send_data()
            sample_data.update_session_length()
            print(f"Round: {i}")
            time.sleep(SLEEP)
        time.sleep(3600)

    print("Done")