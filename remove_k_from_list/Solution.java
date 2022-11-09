import java.util.Arrays;
import java.util.Collection;

class ListNode<T> {
    T value;
    ListNode<T> next;

    ListNode(T x) {
        value = x;
    }

    @Override
    public String toString() {
        return value.toString();
    }
}

public class Solution {
    static ListNode<Integer> solution(ListNode<Integer> l, int k) {
        // Make sure the first node doesn't have a value equal to k
        while (l.value == k) {
            if (l.next != null) {
                l = l.next;
            } else {
                return null;
            }
        }

        // Iterate over the linked list and remove all values equal to k
        ListNode<Integer> node = l;
        while (node.next != null) {
            while (node.next.value == k) {
                node.next = node.next.next;
            }

            // Next node
            node = node.next;
        }

        return l;
    }

    public static void main(String[] args) {
        // Test the solution
        Integer[] integers = {3, 1, 2, 3, 4, 5};
        ListNode<Integer> head = generateLinkedList(Arrays.asList(integers));
        head = solution(head, 3);
        for (ListNode<Integer> node = head; node != null; node = node.next) {
            System.out.println(node);
        }
    }

    private static <T> ListNode<T> generateLinkedList(Collection<T> collection) {
        // Ensure the collection is not empty
        if (collection.isEmpty()) return new ListNode<T>(null);

        // Iterate over the collection and generate the linked list
        ListNode<T> head = null, current = null, next;
        boolean isFirst = true;
        for (T item : collection) {
            // Use the first item of the collection for the head
            if (isFirst) {
                // Create the head node
                head = new ListNode<T>(item);
                current = head;
                isFirst = false;
                continue;
            }

            // Create the next node
            next = new ListNode<T>(item);
            current.next = next;
            current = next;
        }

        return head;
    }
}