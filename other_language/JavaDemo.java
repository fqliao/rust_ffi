
public class JavaDemo {

    public static native int add(int a, int b);
    public static native String hello(String str);
    public static native Person getPerson(String name, int age);

    static {
        System.loadLibrary("rust");
    }

    public static void main(String[] args){

        System.out.println("add: " + JavaDemo.add(1, 2));
        System.out.println("\\nhello: " + JavaDemo.hello("World"));
        System.out.println("\\ngetPerson: " + JavaDemo.getPerson("Alice", 12));
    }
}