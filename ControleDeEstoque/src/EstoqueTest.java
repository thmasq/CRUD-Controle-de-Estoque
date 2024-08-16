import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertTrue;

import org.junit.Test;

import Modelo.Estoque;

public class EstoqueTest {
	
	/**
	 * Teste para verificar o construtor sem parâmetros da classe Estoque.
	 * Verifica se os valores dos atributos empilhadeiradisponivel e caminhaoDentro são inicializados corretamente como zero.
	 * Verifica também se a lista de itens é criada e está vazia.
	 */
	
	@Test
    public void testConstructorWithoutParameters() {
        Estoque estoque = new Estoque();

        assertEquals(0, estoque.getEmpilhadeiradisponivel());
        assertEquals(0, estoque.getCaminhaoDentro());
        assertNotNull(estoque.getItens());
        assertTrue(estoque.getItens().isEmpty());
    }

}
