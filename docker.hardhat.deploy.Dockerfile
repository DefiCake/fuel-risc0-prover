FROM node:18

WORKDIR /app
COPY package.json yarn.lock /app/

RUN yarn install --frozen-lockfile

COPY tsconfig.json hardhat.config.ts /app/
COPY tasks/ /app/tasks/
COPY contracts/ /app/contracts/

RUN ["yarn", "compile"]

COPY deploy/ /app/deploy/
COPY /res/IMAGE_ID /app/res/

CMD ["yarn", "hardhat:deploy", "--network", "anvil", "--reset"]