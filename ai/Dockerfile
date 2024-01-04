# Use an official Python runtime as a parent image
FROM python:3.9-slim

# Set the working directory in the container
WORKDIR /usr/src/app

# Copy the current directory contents into the container at /usr/src/app
COPY . .

RUN pip install --upgrade pip

RUN pip install tensorflow keras protobuf

# Install any needed packages specified in requirements.txt
RUN pip install --no-cache-dir -r requirements.txt

# Port defined in the flask_config.json
EXPOSE 4242

# Run your Flask application script when the container launches
CMD ["python", "flask_main.py"]
