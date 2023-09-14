# This dockerfile is only used for debugging, no actual development
FROM node:18

COPY . .

RUN yarn install --frozen-lockfile

RUN ["yarn", "compile"]

CMD ["npx", "hardhat", "node", "--hostname", "0.0.0.0"]