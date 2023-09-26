import React,{useState} from 'react';
import {
  Box,
  Heading,
  Image,
  Button,
  Text,
  Divider,
  HStack,
  Tag,
  Wrap,
  WrapItem,
  useColorModeValue,
  Container,
  VStack,
  Modal,
  ModalOverlay,
  ModalContent,
  ModalHeader,
  ModalFooter,
  ModalBody,
  ModalCloseButton,
  useDisclosure,
  FormControl,
  FormLabel,
  Input,
  Textarea,
  InputGroup,
  InputRightElement,
  CheckIcon,
  InputLeftElement,
} from '@chakra-ui/react';

import { Link } from "react-router-dom";
import Select from 'react-select';

function generateTo(JobId) {
  return {
    pathname: `/job/${JobId}`,
  };
}



function JobTags(props) {
  const { marginTop = 0, tags } = props;

  const tagElements = tags.map((tag) => {
    return (
      <Tag size={'md'} variant="solid" colorScheme="orange" key={tag}>
        {tag}
      </Tag>
    );
  });

  return <HStack spacing={2} marginTop={marginTop}>{tagElements}</HStack>;
}

const linkStyle = {
  textDecoration: 'none', // Remove underline
  color: 'blue', // Change the text color to blue
  // Add any other styles you want here
};

function JobOwner(props) {
  return (
    <HStack marginTop="2" spacing="2" display="flex" alignItems="center">
      <Image
        borderRadius="full"
        boxSize="40px"
        src="https://100k-faces.glitch.me/random-image"
        alt={`Avatar of ${props.name}`}
      />
      <Text fontWeight="medium">{props.name}</Text>
      <Text>—</Text>
      <Text>{props.date.toLocaleDateString()}</Text>
    </HStack>
  );
}

function JobList() {

  const { isOpen, onOpen, onClose } = useDisclosure()
  
  const initialRef = React.useRef(null)
  const finalRef = React.useRef(null)

  const skillOptions = [
    { value: "html", label: "HTML" },
    { value: "css", label: "CSS" },
    { value: "javascript", label: "JavaScript" },
    { value: "react", label: "React" },
    { value: "python", label: "Python" },
    { value: "java", label: "Java" },
    { value: "csharp", label: "C#" },
    { value: "ruby", label: "Ruby" },
    // Add more programming-related skills as needed
  ];

  const [selectedSkills, setSelectedSkills] = useState([]);

  const handleSkillChange = (selectedOptions) => {
    setSelectedSkills(selectedOptions);
  };

  return (
    <Container maxW={'7xl'} p="12">
      <>
            <Button onClick={onOpen}>Post Job</Button>
            <Modal
              initialFocusRef={initialRef}
              finalFocusRef={finalRef}
              isOpen={isOpen}
              onClose={onClose}
            >
            {/* pub job_id: u128,
            pub account_id: AccountId,
            pub project_title: String,
            pub project_description: String,
            pub project_duration: String,
            pub project_budget: u128,
            pub skill_requirements: Vec<String>,
            pub images: Vec<String>,
            pub bid_available: bool, */}

              <ModalOverlay />
              <ModalContent>
                <ModalHeader>Post Job</ModalHeader>
                <ModalCloseButton />
                <ModalBody pb={6}>
                  <FormControl>
                    <FormLabel>Title</FormLabel>
                    <Input ref={initialRef} placeholder='title' />
                  </FormControl>

                  <FormControl mt={4}>
                    <FormLabel>Description</FormLabel>
                    <>
                      <Textarea
                        // value={}
                        // onChange={handleInputChange}
                        placeholder='job description'
                        size='sm'
                      />
                    </>
                  </FormControl>

                  <FormControl>
                    <FormLabel>Duration</FormLabel>
                    <Input ref={initialRef} placeholder='duration in hours' />
                  </FormControl>

                  <FormControl>
                  <FormLabel>Budget</FormLabel>
                      <Input  type='number' placeholder='Enter amount' />
                  </FormControl>

                  <FormControl>
                  <div>
                    <FormLabel>Skills Reguired</FormLabel>
                    <Select
                      isMulti
                      options={skillOptions}
                      value={selectedSkills}
                      onChange={handleSkillChange}
                      placeholder="Select skills..."
                    />
                  </div>
                  </FormControl>
                  <FormControl>
                  <div style={{padding:10}}>
                    <input type="file" multiple/>
                  </div>
                  </FormControl>
                </ModalBody>
                <ModalFooter>
                  <Button colorScheme='blue' mr={3}>
                    Post
                  </Button>
                  <Button onClick={onClose}>Cancel</Button>
                </ModalFooter>
              </ModalContent>
            </Modal>
          </>
      <Heading as="h5"  padding="0.5rem 0 0">Jobs</Heading>
      <Box
        marginTop={{ base: '1', sm: '5' }}
        display="flex"
        flexDirection={{ base: 'column', sm: 'row' }}
        justifyContent="space-between">
        <Box
          display="flex"
          flex="1"
          marginRight="3"
          position="relative"
          alignItems="center">
          <Box
            width={{ base: '100%', sm: '85%' }}
            zIndex="2"
            marginLeft={{ base: '0', sm: '5%' }}
            marginTop="5%">
            <Box textDecoration="none" _hover={{ textDecoration: 'none' }}>
              <Image
                borderRadius="lg"
                src={
                  'https://images.unsplash.com/photo-1499951360447-b19be8fe80f5?ixlib=rb-1.2.1&ixid=MXwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHw%3D&auto=format&fit=crop&w=800&q=80'
                }
                alt="some good alt text"
                objectFit="contain"
              />
            </Box>
          </Box>
          <Box zIndex="1" width="100%" position="absolute" height="100%">
            <Box
              bgGradient={useColorModeValue(
                'radial(orange.600 1px, transparent 1px)',
                'radial(orange.300 1px, transparent 1px)',
              )}
              backgroundSize="20px 20px"
              opacity="0.4"
              height="100%"
            />
          </Box>
        </Box>
        <Box
          display="flex"
          flex="1"
          flexDirection="column"
          justifyContent="center"
          marginTop={{ base: '3', sm: '0' }}>
          <JobTags tags={['Engineering', 'Product']} />
          <Heading marginTop="1" fontSize={15}>
            <Text textDecoration="none" _hover={{ textDecoration: 'none' }}>
              Job title
            </Text>
          </Heading>
          <Text
            as="p"
            marginTop="2"
            color={useColorModeValue('gray.700', 'gray.200')}
            fontSize="lg">
            Lorem Ipsum is simply dummy text of the printing and typesetting industry.
            Lorem Ipsum has been the industry&apos;s standard dummy text ever since the
            1500s, when an unknown printer took a galley of type and scrambled it to make
            a type specimen book.
          </Text>
          <Link to={generateTo(1)} style={linkStyle}>
            View
          </Link>
          <JobOwner name="John Doe" date={new Date('2021-04-06T19:01:27Z')} />
        </Box>
      </Box>
    </Container>
  );
}

export default JobList;
