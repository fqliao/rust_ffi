public class Person {
    // field must be public for rust
    private String name;
    private int age;

    @Override
    public String toString() {
        return "{" +
                "name=\"" + name + '\"' +
                ", age=" + age +
                '}';
    }
}