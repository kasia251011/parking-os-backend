# Use the official Python base image
FROM python:3.9

# Set the working directory in the container
WORKDIR /app

# Copy the current directory contents into the container
COPY . /app

# Install any needed packages specified in requirements.txt
RUN pip install --no-cache-dir -r requirements.txt

# Port number the container should expose
EXPOSE 27017

# Set the entry point for the container
CMD [ "python3", "load_data_mongodb.py" ]
