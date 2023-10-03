import React, { useEffect, useState } from 'react';

import {
  Box,
  chakra,
  Container,
  Stack,
  Text,
  Image,
  Flex,
  VStack, // Import VStack for vertical stacking
  Button,
  Heading,
  SimpleGrid,
  StackDivider,
  useColorModeValue,
  VisuallyHidden,
  List,
  ListItem,
  Tabs,
  TabList, 
  TabPanels,
  Tab,
  TabPanel,
} from '@chakra-ui/react';
import { MdLocalShipping } from 'react-icons/md';
import * as nearAPI from "near-api-js";

export default function ProfileTabs({ isSignedIn, wallet ,contractId}) {

  console.log(wallet.walletSelector.options.network);
  const [freelancer,setFreelancer] = useState(null);
  const [portfolios,setPortfolios] = useState([]);
  const [experiences,setExperiences] = useState([]);
  const [bal, setBalance] = useState("");

  const { keyStores } = nearAPI;
  const { connect } = nearAPI;
  const myKeyStore = new keyStores.BrowserLocalStorageKeyStore();


  const connectionConfig = {

    networkId: "testnet",
    keyStore: myKeyStore,
    nodeUrl: "https://rpc.testnet.near.org",
    walletUrl: "https://wallet.testnet.near.org",
    helperUrl: "https://helper.testnet.near.org",
    explorerUrl: "https://explorer.testnet.near.org",
  };


  
  const near = "1000000000000000000000000";




  useEffect(() => {
    // Function to perform the async action
    async function handleAccount() {
      try {
        // Check if the user is signed in
        if (isSignedIn) {
          // Perform the asynchronous action
          const nearConnection = await connect(connectionConfig);
          const account = await nearConnection.account(wallet.accountId);
          const balance = await account.getAccountBalance();
          setBalance(balance);
          getFreelancer().then(setFreelancer);
          getPortfolios().then(setPortfolios);
          getExperiences().then(setExperiences);
          // console.log("alas",balance);
          
          // const account_details = await account.getAccountDetails();
          // console.log(account_details);
          // You can add further logic here based on the result
        } else {
          console.log('User is not signed in.');
        }
      } catch (error) {
        // Handle any errors here
        console.error('Error performing async action:', error);
      }
    }

    // Call the function to handle the async action
    handleAccount();
  }, [isSignedIn,wallet]);

  // pub account_id: AccountId,
  // pub profile_image: String,
  // pub full_name: String,
  // pub hourly_rate: u128,
  // pub profession: String,
  // pub payment_preference: String,
  // pub skills: Vec<String>,
  // pub profile_rating: u128,
  // pub is_profile_public: bool,


  function getFreelancer() {
    const freelancer_account = wallet.accountId;
    return wallet.viewMethod({ method: "get_freelancer", args: {account_id:freelancer_account}, contractId });

  }

  function getPortfolios() {
    const freelancer_account = wallet.accountId;
    return wallet.viewMethod({ method: "get_all_freelancer_portfolios_for_account", args: {account_id:freelancer_account}, contractId });

  }
  function getExperiences() {
    const freelancer_account = wallet.accountId;
    return wallet.viewMethod({ method: "get_all_freelancer_experience_for_account", args: {account_id:freelancer_account}, contractId });

  }


  console.log(portfolios);
  console.log(experiences);

  return (
    <Container maxW={'7xl'}>

    <Tabs variant='unstyled'>
      <TabList>
        <Tab _selected={{ color: 'white', bg: 'blue.500' }} color="black">Account</Tab>
        <Tab _selected={{ color: 'white', bg: 'green.400' }} color="black">Portfolio</Tab>
        <Tab _selected={{ color: 'white', bg: 'blue.600' }} color="black">Experience</Tab>
      </TabList>
      <TabPanels>
        <TabPanel>

        <SimpleGrid
        columns={{ base: 1, lg: 1 }}
        spacing={{ base: 8, md: 10 }}
        py={{ base: 10, md: 10 }}
      >
        <VStack spacing={{ base: 6, md: 6 }}> {/* Wrap your content in VStack */}
          <Box as={'header'} textAlign="left">
          {isSignedIn ? (
            <Heading
                lineHeight={1.1}
                fontWeight={600}
                fontSize={{ base: '2xl', sm: '2xl', lg: '2xl' }}
              >
                {wallet.accountId}
              </Heading>
            ) : (
              <p>Please sign in to see the heading.</p>
            )}
            <Text
              color={useColorModeValue('gray.900', 'gray.400')}
              fontWeight={300}
              fontSize={'2xl'}>
              {(bal.available/near).toFixed(5)}  NEAR
            </Text>
          </Box>
          {freelancer === null ? (
        <p>Please update your details</p>
      ) : freelancer === undefined || freelancer.length === 0 ? (
        <p>Please update your details</p>
      ) : (
        <>
          <Stack
            spacing={{ base: 4, sm: 6 }}
            direction={'column'}
            divider={
              <StackDivider borderColor={useColorModeValue('gray.200', 'gray.600')} />
            }>
            <VStack spacing={{ base: 4, sm: 6 }}>
              <Text
                color={useColorModeValue('gray.500', 'gray.400')}
                fontSize={'2xl'}
                fontWeight={'300'}>
                Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy
                eirmod tempor invidunt ut labore
              </Text>
              <Text fontSize={'lg'}>
                Lorem ipsum dolor sit amet, consectetur adipisicing elit. Ad aliquid amet
                at delectus doloribus dolorum expedita hic, ipsum maxime modi nam officiis
                porro, quae, quisquam quos reprehenderit velit? Natus, totam.
              </Text>
            </VStack>
            <Box>
              <Text
                fontSize={{ base: '16px', lg: '18px' }}
                color={useColorModeValue('yellow.500', 'yellow.300')}
                fontWeight={'500'}
                textTransform={'uppercase'}
                mb={'4'}>
                Features
              </Text>

              <SimpleGrid columns={{ base: 1, md: 2 }} spacing={10}>
                <List spacing={2}>
                  <ListItem>Chronograph</ListItem>
                  <ListItem>Master Chronometer Certified</ListItem>
                  <ListItem>Tachymeter</ListItem>
                </List>
                <List spacing={2}>
                  <ListItem>Anti‑magnetic</ListItem>
                  <ListItem>Chronometer</ListItem>
                  <ListItem>Small seconds</ListItem>
                </List>
              </SimpleGrid>
            </Box>
            <Box>
              <Text
                fontSize={{ base: '16px', lg: '18px' }}
                color={useColorModeValue('yellow.500', 'yellow.300')}
                fontWeight={'500'}
                textTransform={'uppercase'}
                mb={'4'}>
                Product Details
              </Text>

              <List spacing={2}>
                <ListItem>
                  <Text as={'span'} fontWeight={'bold'}>
                    Between lugs:
                  </Text>{' '}
                  20 mm
                </ListItem>
                <ListItem>
                  <Text as={'span'} fontWeight={'bold'}>
                    Bracelet:
                  </Text>{' '}
                  leather strap
                </ListItem>
                <ListItem>
                  <Text as={'span'} fontWeight={'bold'}>
                    Case:
                  </Text>{' '}
                  Steel
                </ListItem>
                <ListItem>
                  <Text as={'span'} fontWeight={'bold'}>
                    Case diameter:
                  </Text>{' '}
                  42 mm
                </ListItem>
                <ListItem>
                  <Text as={'span'} fontWeight={'bold'}>
                    Dial color:
                  </Text>{' '}
                  Black
                </ListItem>
                <ListItem>
                  <Text as={'span'} fontWeight={'bold'}>
                    Crystal:
                  </Text>{' '}
                  Domed, scratch‑resistant sapphire crystal with anti‑reflective treatment
                  inside
                </ListItem>
                <ListItem>
                  <Text as={'span'} fontWeight={'bold'}>
                    Water resistance:
                  </Text>{' '}
                  5 bar (50 metres / 167 feet)
                </ListItem>
              </List>
            </Box>
          </Stack>

          <Button
            rounded={'none'}
            w={'full'}
            mt={8}
            size={'lg'}
            py={'7'}
            bg={useColorModeValue('gray.900', 'gray.50')}
            color={useColorModeValue('white', 'gray.900')}
            textTransform={'uppercase'}
            _hover={{
              transform: 'translateY(2px)',
              boxShadow: 'lg',
            }}>
            Add to cart
          </Button>

          <Stack direction="row" alignItems="center" justifyContent={'center'}>
            <MdLocalShipping />
            <Text>2-3 business days delivery</Text>
          </Stack>
          </>
          )}
        </VStack>
      </SimpleGrid>
      
        </TabPanel>
        <TabPanel>
        {portfolios.length === 0 ? (
        <p>No portfolio found - add</p>
      ) : (
        <div>
          {/* Display freelancer info */}
          <p>Name: {freelancer.name}</p>
          <p>Email: {freelancer.email}</p>
          {/* Add more info fields as needed */}
        </div>
      )}
        </TabPanel>
        <TabPanel>
        {experiences.length === 0 ? (
        <p>no experiences found - add</p>
      ) : (
        <div>
          {/* Display freelancer info */}
          <p>Name: {freelancer.name}</p>
          <p>Email: {freelancer.email}</p>
          {/* Add more info fields as needed */}
        </div>
      )}
        </TabPanel>
      </TabPanels>
    </Tabs>

    </Container>
  );
}
