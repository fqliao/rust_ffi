
public class JavaDemo {

    public static native int add(int a, int b);
//    public static native Person getPerson(String name, int age);

    static {
        System.loadLibrary("rust");
    }

    public static void main(String[] args){

        System.out.println("add: " + JavaDemo.add(1, 2));
//        System.out.println(java_demo.getPerson("alice", 12));
    }
}