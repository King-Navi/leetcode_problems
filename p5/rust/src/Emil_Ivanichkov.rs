pub fn longest_palindrome(s: String) -> String {
    if s.len() < 2 {
        return s;
    };

    let chars: Vec<char> = s.chars().collect();
    let mut normalized: Vec<char> = Vec::new();
    normalized.push('$');
    normalized.push('#');
    chars.iter().for_each(|c| {
        normalized.push(*c);
        normalized.push('#');
    });
    normalized.push('!');

    let mut center = 0;
    let mut right_boundary = 0;
    let mut radiuses = vec![0; normalized.len()];

    for i in 1..normalized.len() - 1 {
        let mirror = 2 * center as isize - i as isize;

        if i < right_boundary {
            radiuses[i] = (right_boundary - i).min(radiuses[mirror as usize])
        }

        while normalized[i - radiuses[i] - 1] == normalized[i + radiuses[i] + 1] {
            radiuses[i] += 1;
        }

        if i + radiuses[i] > right_boundary {
            center = i;
            right_boundary = i + radiuses[i]
        }
    }

    let (index, max_radius) = radiuses
        .iter()
        .enumerate()
        .max_by_key(|(_, value)| *value)
        .unwrap();

    let start = (index - max_radius) / 2;
    
    chars[start..(start + max_radius)]
        .iter()
        .collect::<String>()
}