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
  Badge,
  Divider,
} from '@chakra-ui/react'
import { MdLocalShipping } from 'react-icons/md'
import { useParams} from 'react-router-dom';
import React,{useState,useEffect} from 'react';

export default function JobDetails({ isSignedIn, wallet , contractId}) {

  const [job, setJob] = useState([]);

  const params = useParams();

  useEffect(() => {
  
    getJob().then(setJob);

  }
  , []);


  function getJob() {
    console.log(contractId)
    const job_id = Number(params.id);
    return wallet.viewMethod({ method: "get_client_job", args: {job_id:job_id}, contractId });

  }

 


  console.log(job);

  return (
    <Container maxW={'7xl'}>
      <SimpleGrid
        columns={{ base: 1, lg: 1 }}
        spacing={{ base: 8, md: 10 }}
        py={{ base: 18, md: 24 }}>
        <Stack spacing={{ base: 6, md: 10 }}>
          <Box as={'header'}>
            <Heading
              lineHeight={1.1}
              fontWeight={600}
              fontSize={{ base: '2xl', sm: '4xl', lg: '5xl' }}>
              {job.project_title}
            </Heading>
            <Text
              color={useColorModeValue('gray.900', 'gray.400')}
              fontWeight={300}
              fontSize={'2xl'}>
              {job.project_budget} NEAR
            </Text>
          </Box>

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
                {job.account_id}
              </Text>
              <Text fontSize={'lg'}>
                {job.project_description}
              </Text>
            </VStack>
            <Box>
              <Text
                fontSize={{ base: '16px', lg: '18px' }}
                color={useColorModeValue('yellow.500', 'yellow.300')}
                fontWeight={'500'}
                textTransform={'uppercase'}
                mb={'4'}>
                Skill Requirements
              </Text>

              <SimpleGrid columns={{ base: 1, md: 2 }} spacing={10}>
              <List spacing={2}>
                  {job && job.skill_requirements ? (
                    job.skill_requirements.map((skill, skillIndex) => (
                      <div key={skillIndex}>
                        <ListItem>{skill}</ListItem>
                      </div>
                    ))
                  ) : (
                    // Render a placeholder or loading message when job is undefined or skill_requirements is missing
                    <div>Loading...</div>
                  )}
              </List>

              <List spacing={2}>
              <Text
                fontSize={{ base: '12px', lg: '12px' }}
                color={useColorModeValue('yellow.500', 'yellow.300')}
                fontWeight={'500'}
                textTransform={'uppercase'}
                mb={'4'}>
                Files
              </Text>

                {job && job.images && job.images.map((file, fileIndex) => (
                  <ListItem key={fileIndex}>
                    <a href={file} target="_blank" rel="noopener noreferrer" style={{ color: 'blue', textDecoration: 'underline' }}>
                      File {fileIndex + 1}
                    </a>
                  </ListItem>
                ))}
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
                    Project Duration:
                  </Text>{' '}
                  {job.project_duration} hours
                </ListItem>
                <ListItem>
                {job.bid_available ? (
                  <div></div>
                  ) : (
                    // Render the "Removed" badge when the job is not available
                    <span style={{paddingLeft:'3'}}>
                      <Badge colorScheme='red'>Not Available</Badge>
                    </span>
                  )}
                </ListItem>

              </List>
            </Box>
          </Stack>

        </Stack>
      </SimpleGrid>
      
    </Container>
  )
}