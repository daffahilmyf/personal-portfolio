import { render, screen } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import { describe, expect, test, vi } from 'vitest';

import { Button } from '$components';

describe('button component', () => {
  test('button default visible', () => {
    render(Button);
    const button = screen.getByRole('button');

    expect(button).toBeInTheDocument();
    expect(button).toHaveTextContent('Button');
  });

  test('button onClick success', async () => {
    const user = userEvent.setup();
    const onClick = vi.fn();

    const { component } = render(Button);
    component.$on('click', onClick);

    const button = screen.getByRole('button');
    await user.click(button);

    expect(onClick).toHaveBeenCalledOnce();
  });
});
