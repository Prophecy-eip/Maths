# Use an official Python runtime as a parent image
FROM python:3.9-slim

# Port defined in the flask_config.json
EXPOSE 4242

# Set the working directory in the container
WORKDIR /usr/src/app

# Copy the current directory contents into the container at /usr/src/app
COPY . .

RUN pip install --upgrade pip

# Install any needed packages specified in requirements.txt
RUN pip install --no-cache-dir -r requirements.txt

# Run your Flask application script when the container launches
CMD ["python", "flask_main.py"]
