from enum import Enum
import random
import pymongo
import os
from dotenv import load_dotenv
from bson.objectid import ObjectId
import requests
from datetime import datetime, timedelta

load_dotenv()

MONGO_URI = os.getenv("MONGO_URI")
MONGO_DB_NAME = os.getenv("MONGO_DB_NAME")

url = 'https://parking-os-backend.onrender.com'
parking_lot_endpoint = '/parking-lots'
parking_spots_endpoint = '/parking-spots'

def generate_random_timestamp_2023():
    year = 2023
    month = random.randint(1, 12)
    day = random.randint(1, 28)  # Assuming all months have at most 28 days for simplicity
    hour = random.randint(0, 23)
    minute = random.randint(0, 59)
    second = random.randint(0, 59)
    
    timestamp = int(datetime(year, month, day, hour, minute, second).timestamp())
    return timestamp

# Connect to MongoDB
client = pymongo.MongoClient(MONGO_URI)
db = client[MONGO_DB_NAME]

# Clear all data from all collections
for collection_name in db.list_collection_names():
    db[collection_name].delete_many({})

# Fill collections with new data
db["user"].insert_many([
    {
        "_id": ObjectId("5f9b3b9b9d9b9d9b9d9b9d9b"),
        "name": "Krzysztof",
        "surname": "Admin",
        "account_balance": 0,
        "blocked": False,
        "email": "krzysztof@admin.com",
        "password": "$2a$10$p0MGb/vlu.sjfHiCV.k02.Bq14NAigmWbNqDJ19R8h0UBGf.UwcZy", # admin123
        "role": "Admin"
    },
    {
        "_id": ObjectId("5f9b3b9b9d9b9d9b9d9b9d9c"),
        "name": "Jan",
        "surname": "Kowalski",
        "account_balance": 1000,
        "blocked": False,
        "email": "jan.kowalski@example.com",
        "password": "$2a$10$v/cJp3VcYXwG.3xfUerXFOYoqBxn1Fc04wQuKug2Nm/ev46u.u8SS", # password123
        "role": "User"
    },
    {
        "_id": ObjectId("5f9b3b9b9d9b9d9b9d9b9d9d"),
        "name": "Adam",
        "surname": "Nowak",
        "account_balance": 2000,
        "blocked": False,
        "email": "adam.nowak@example.com",
        "password": "$2a$10$YKYPj5.wk6lpG9b7PazFtO0WMB.D.QQZDnVj9r40Z1Ume5hRbpIQK", # secure123
        "role": "User"
    },
    {
        "_id": ObjectId("5f9b3b9b9d9b9d9b9d9b9d9e"),
        "name": "Anna",
        "surname": "Kowalska",
        "account_balance": 3000,
        "blocked": False,
        "email": "anna.kowalska@example.com",
        "password": "$2a$10$YQRpqbv2gZjchpc4gymo3Ok92PpkEsPXgpco7cSoi/wrIgGBWSifu", # pass123
        "role": "User"
    },
    {
        "_id": ObjectId("5f9b3b9b9d9b9d9b9d9b9d9f"),
        "name": "Jan",
        "surname": "Nowak",
        "account_balance": 4000,
        "blocked": False,
        "email": "jan.nowak@example.com",
        "password": "$2a$10$VfvVXxOo39T5zsJCfCXQ4uktwn9KF5OVvgZBSg/6458MuHRHqzZ9C", # test123
        "role": "User"
    },
    {
        "_id": ObjectId("5f9b3b9b9d9b9d9b9d9b9da0"),
        "name": "Paweł",
        "surname": "Kowalski",
        "account_balance": 5000,
        "blocked": True,
        "email": "pawel.kowalski@example.com",
        "password": "$2a$10$Z9/xJYUZFPcS54UuAETIGu4RFsskaOEel9NOjL1kysrMyKp0cvjUm", # hello123
        "role": "User"
    },
    {
        "_id": ObjectId("5f9b3b9b9d9b9d9b9d9b9da1"),
        "name": "Grzegorz",
        "surname": "Witkiewicz",
        "account_balance": 6000,
        "blocked": True,
        "email": "grzegorz.witkiewicz@example.com",
        "password": "$2a$10$jC6LzGqaX9RM32QdZj.hMeqcUtbnnZPBNkTRSfVH9oOHlax1ufkXu", # grze123
        "role": "User"
    }
])

db["vehicle"].insert_many([
    {"_id": ObjectId("5f9b3b9b9d9b9d9b9d9b9d90"), "user_id": "5f9b3b9b9d9b9d9b9d9b9d9c", "type": "Car", "brand": "BMW", "model": "X5", "license_plate_number": "WAW12345"},
    {"_id": ObjectId("5f9b3b9b9d9b9d9b9d9b9d91"), "user_id": "5f9b3b9b9d9b9d9b9d9b9d9c", "type": "Car", "brand": "Mercedes", "model": "Actros", "license_plate_number": "WAW54321"},
    {"_id": ObjectId("5f9b3b9b9d9b9d9b9d9b9d92"), "user_id": "5f9b3b9b9d9b9d9b9d9b9d9d", "type": "Car", "brand": "Audi", "model": "A6", "license_plate_number": "WAW67890"},
    {"_id": ObjectId("5f9b3b9b9d9b9d9b9d9b9d93"), "user_id": "5f9b3b9b9d9b9d9b9d9b9d9e", "type": "Car", "brand": "BMW", "model": "X5", "license_plate_number": "WAW12345"},
    {"_id": ObjectId("5f9b3b9b9d9b9d9b9d9b9d94"), "user_id": "5f9b3b9b9d9b9d9b9d9b9d9f", "type": "Car", "brand": "Audi", "model": "A6", "license_plate_number": "WAW67890"},
])

headers = { 'Content-Type': 'application/json' }
data_parking_lot = [
    {
        "costOfMaintenance": {"electricity": 400, "cleaning": 500, "security": 1000}, 
        "location": {"city": "Warszawa", "address": "Al. Jerozolimskie 54, 00-024 Warszawa", "latitude": 52.228668, "longitude": 21.003550}, 
        "levels": [{"cars": 10, "trucks": 5}, {"cars": 10, "trucks": 5}, {"cars": 10, "trucks": 5}, {"cars": 10, "trucks": 5}],
        "tariffs": [{"minTime": 1, "maxTime": 8, "pricePerHour": 5.0}, {"minTime": 9, "maxTime": 100, "pricePerHour": 4.0}]
    },
    {
        "costOfMaintenance": {"electricity": 400, "cleaning": 600, "security": 1300}, 
        "location": {"city": "Warszawa", "address": "ul. Hoża 84, 00-682 Warszawa", "latitude": 52.226170, "longitude": 21.013750}, 
        "levels": [{"cars": 15, "trucks": 5}, {"cars": 10, "trucks": 5}, {"cars": 5, "trucks": 2}],
        "tariffs": [{"minTime": 1, "maxTime": 5, "pricePerHour": 4.0}, {"minTime": 6, "maxTime": 100, "pricePerHour": 3.0}]
    },
    {
        "costOfMaintenance": {"electricity": 500, "cleaning": 700, "security": 1500}, 
        "location": {"city": "Warszawa", "address": "ul. Furmańska 14, 00-313 Warszawa", "latitude": 52.244260, "longitude": 21.019190}, 
        "levels": [{"cars": 7, "trucks": 3}, {"cars": 7, "trucks": 3}],
        "tariffs": [{"minTime": 1, "maxTime": 100, "pricePerHour": 8.0}]
    },
]

for parking_lot in data_parking_lot:
    response = requests.post(url+parking_lot_endpoint, json=parking_lot, headers=headers)
    if response.status_code == 422:
        print("Error 422:")
        print(response.content)
    else:
        print(response.status_code)

parking_lot_array = requests.get(url+parking_lot_endpoint).json()
print("Parking lot array:" + str(parking_lot_array))
parking_lot_id = parking_lot_array[0]["id"].replace("(", "").replace(")", "").replace("'", "").replace(",", "")

get_parking_spots_by_parking_lot_id = url+parking_lot_endpoint+"/"+str(parking_lot_id)+parking_spots_endpoint
get_parking_spots_by_parking_lot_id = get_parking_spots_by_parking_lot_id
print("parking spots endpoint ", get_parking_spots_by_parking_lot_id)
parking_spot_array = requests.get(get_parking_spots_by_parking_lot_id).json()
print("Parking spot array:" + str(parking_spot_array))

user_ids = [
    ObjectId("5f9b3b9b9d9b9d9b9d9b9d9b"),
    ObjectId("5f9b3b9b9d9b9d9b9d9b9d9c"),
    ObjectId("5f9b3b9b9d9b9d9b9d9b9d9d"),
    ObjectId("5f9b3b9b9d9b9d9b9d9b9d9e"),
    ObjectId("5f9b3b9b9d9b9d9b9d9b9d9f"),
]

vehicle_license_number = [
    "WAW12345",
    "WAW54321",
    "WAW67890",
    "WAW12345",
    "WAW67890",
]

tickets = []
for i in range(1000):
    start_timestamp = generate_random_timestamp_2023()
    end_timestamp = start_timestamp + int(timedelta(hours=random.randint(1, 10)).total_seconds())
    time_difference_hours = (end_timestamp - start_timestamp) / 3600
    amount_paid = round(time_difference_hours * random.uniform(5, 7))

    tickets.append({
        "_id": ObjectId(),
        "user_id": str(user_ids[i % len(user_ids)]),
        "vehicle_license_number": vehicle_license_number[i % len(vehicle_license_number)],
        "parking_spot_id": parking_spot_array[int(i % len(parking_spot_array))]["id"],
        "issue_timestamp": start_timestamp,
        "end_timestamp": end_timestamp,
        "amount_paid": amount_paid,
        "level": parking_spot_array[int(i % len(parking_spot_array))]["level"],
        "spot_ordinal_number": random.randint(0, 4),
        "parking_lot_id": str(parking_lot_id),
        "code": f"CODE{i}",
    })

# Add the generated tickets to the "ticket" collection
db["ticket"].insert_many(tickets)

# Close the MongoDB connection
client.close()