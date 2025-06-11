FROM node:18-slim

# Create app directory
WORKDIR /app

# Install app dependencies
# A wildcard is used to ensure both package.json AND package-lock.json are copied
# where available (npm@5+)
COPY package*.json ./

RUN npm install

# Bundle app source
COPY . .

# Copy the service account key file
COPY key.json .

# Set the GOOGLE_APPLICATION_CREDENTIALS environment variable
ENV GOOGLE_APPLICATION_CREDENTIALS="/app/key.json"

# Set the PORT environment variable
ENV PORT 8080

# Set the PORT environment variable
ENV PORT 8080

# Start the app
CMD [ "node", "index.js" ]