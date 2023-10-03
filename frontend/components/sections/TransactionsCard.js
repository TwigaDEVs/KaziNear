'use client'

import {
  Box,
  chakra,
  Container,
  Stack,
  Text,
  Image,
  Flex,
  VStack,
  Button,
  Heading,
  SimpleGrid,
  StackDivider,
  useColorModeValue,
  VisuallyHidden,
  List,
  ListItem,
  Stat,
  StatLabel,
  StatNumber,
  StatHelpText,
  StatArrow,
  StatGroup,
} from '@chakra-ui/react'
import { MdLocalShipping } from 'react-icons/md'
import React, { useEffect, useState } from 'react';

export default function TransactionsCard({ isSignedIn, wallet ,contractId}) {

  const [transactions,setTransactions] = useState([]);

  useEffect(() => {
  
    getTransactions().then(setTransactions);
    // newConnectBalance.nearConnect().then(setAccBalance);
    // viewProfile().then((data) => (setUserProfile(data)));
    // ;

  }
  , []);

  function getTransactions() {
		console.log(contractId)
    const user_account = wallet.accountId;
    return wallet.viewMethod({ method: "get_transactions_for_account", args: {account_id:user_account}, contractId });

	
	  }

     console.log(transactions);

  return (
    <Container maxW={'7xl'}>
      <SimpleGrid
        columns={{ base: 1, lg: 1 }}
        spacing={{ base: 8, md: 10 }}
        py={{ base: 10, md: 10 }}>
        <Stack spacing={{ base: 6, md: 6 }}>
          <Box as={'header'}>
            <Heading
              lineHeight={1.1}
              fontWeight={600}
              fontSize={{ base: '2xl', sm: '2xl', lg: '2xl' }}>
              All Transactions
            </Heading>
          </Box>

          <Stack
            spacing={{ base: 4, sm: 6 }}
            direction={'column'}
            divider={
              <StackDivider borderColor={useColorModeValue('gray.200', 'gray.600')} />
            }>
            <Box>
              <Text
                fontSize={{ base: '16px', lg: '18px' }}
                color={useColorModeValue('yellow.500', 'yellow.300')}
                fontWeight={'500'}
                textTransform={'uppercase'}
                mb={'4'}>
                --------------
              </Text>

              <SimpleGrid columns={{ base: 1, md: 2 }} spacing={10}>
                <Stat>
                    <StatLabel>Collected Fees</StatLabel>
                    <StatNumber>£0.00</StatNumber>
                    <StatHelpText>Feb 12 - Feb 28</StatHelpText>
                </Stat>
              </SimpleGrid>
            </Box>
            <Box>
 
            </Box>
          </Stack>

          <Stack direction="row" alignItems="center" justifyContent={'center'}>
            <MdLocalShipping />
            <Text>2-3 business days delivery</Text>
          </Stack>
        </Stack>
      </SimpleGrid>
    </Container>
  )
}