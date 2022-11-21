import scala.collection.mutable.Stack

def solution(a: Option[ListNode[Int]], b: Option[ListNode[Int]]): Option[ListNode[Int]] =
  // Function for safely pushing a node's value to a stack
  val pushNode = (stack: Stack[Int], node: Option[ListNode[Int]]) => {
    node match
      case Some(other) => {
        stack.push(node.get.value)
        node.get.next
      }
      case None => None
  }

  // Use two stacks to invert the linked lists
  var stackA = Stack[Int]()
  var stackB = Stack[Int]()
  var nodeA = a
  var nodeB = b
  while nodeA.isDefined || nodeB.isDefined do
    nodeA = pushNode(stackA, nodeA)
    nodeB = pushNode(stackB, nodeB)

  // Pop elements from the stacks and add them together to form the result
  var resultStack = Stack[Int]()
  var surplus = 0
  while !stackA.isEmpty || !stackB.isEmpty do
    val result: Int = (if stackA.isEmpty then 0 else stackA.pop) + (if stackB.isEmpty then 0 else stackB.pop) + surplus
    resultStack.push(result % 10000)
    surplus = result / 10000

  // Add the surplus if needed
  if surplus > 0 then
    resultStack.push(surplus)
    
  // Assemble the result linked list from the result stack
  val head = Some(ListNode(0))
  var node = head
  while !resultStack.isEmpty do
    val newNode = Some(ListNode(resultStack.pop))
    node.get.next = newNode
    node = newNode

  head.get.next

// Generate a linked list from an array
def generateListFromArray(array: Array[Int]): Option[ListNode[Int]] =
  if array.length == 0 then return None

  // Iterate over the array and construct list nodes
  val head = Some(ListNode(array(0)));
  var node = head
  for i <- 1 to array.length - 1 do
    val newNode = Some(ListNode(array(i)))
    node.get.next = newNode
    node = newNode

  head

// Generate an array from a linked list
def generateArrayFromList(list: Option[ListNode[Int]]): Array[Int] =
  var array = Array[Int]()
  var node = list
  while node.isDefined do
    array = array :+ node.get.value
    node = node.get.next

  array

@main def testSolution(): Unit =
  // Test the helper functions
  testGenerateListFromArrayFunction()
  testGenerateArrayFromListFunction()

  // Test the solution
  var a = Array(9876, 5432, 1999)
  var b = Array(1, 8001)
  testAddition(
    Array(9876, 5434, 0),
    a,
    b
  )

  a = Array(123, 4, 5)
  b = Array(100, 100, 100)
  testAddition(
    Array(223, 104, 105),
    a,
    b
  )

  a = Array(1)
  b = Array(9999, 9999, 9999, 9999, 9999, 9999)
  testAddition(
    Array(1, 0, 0, 0, 0, 0, 0),
    a,
    b
  )

// Test the generateListFromArray helper function
def testGenerateListFromArrayFunction(): Unit =
  val list = generateListFromArray(Array(1, 2))
  assert(list.isDefined)
  assert(list.get.value == 1, s"Actual value: ${list.get.value} != 1")
  assert(list.get.next.isDefined)
  assert(list.get.next.get.value == 2, s"Actual value: ${list.get.next.get.value} != 2")

// Test the generateArrayFromList helper function
def testGenerateArrayFromListFunction(): Unit =
  val list = generateListFromArray(Array(1, 2))
  val array = generateArrayFromList(list)
  assert(array.sameElements(Array(1, 2)), s"${arrayToString(array)} != [1, 2]")

// Test a single addition
def testAddition(expected: Array[Int], a: Array[Int], b: Array[Int]): Unit =
  // Print debug message
  println(s"Testing ${arrayToString(a)} + ${arrayToString(b)} = ${arrayToString(expected)}")

  // Convert arrays into linked lists
  val first = generateListFromArray(a)
  val second = generateListFromArray(b)

  // Calculate the solution and convert the result into an array
  val actual = generateArrayFromList(solution(first, second))
  assert(expected.sameElements(actual), s"The actual result ${arrayToString(actual)} is not the same as the expected result ${arrayToString(expected)}.")

// Get a string representation of an array
def arrayToString(array: Array[Int]): String =
  var st = "["
  for i <- array do
    st += i + ", "

  // Trim the string's trailing comma if needed
  if !array.isEmpty then st = st.substring(0, st.length - 2)
  st + ']'

// Check if the array has space for more items
//     if usedSize == array.size then
//       // Resize the array
//       var newArray = Array[Int](usedSize + 10)
//       array.copyToArray(newArray)
//       array = newArray
