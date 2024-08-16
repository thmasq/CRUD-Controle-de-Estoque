import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;

import org.junit.Test;

import Controle.ControleEstoque;
import Modelo.Eletronico;
import Modelo.Movel;

public class ControleDeEstoqueTest {
	
	/**
	 * Teste para verificar o salvamento de móveis no controle de estoque.
	 * Verifica se um móvel é salvo corretamente no estoque e se a lista de móveis é atualizada corretamente.
	 */
	
    @Test
    public void testSalvarMoveis() {
        Movel movel = new Movel("Cadeira", "Madeira", "", "", 4, 5, 3, 2, "");
        ControleEstoque controle = new ControleEstoque();

        boolean resultado = controle.salvarmoveis(movel);

        assertTrue(resultado);
        assertEquals(1, controle.retornamoveis().size());
        assertTrue(controle.retornamoveis().contains(movel));
    }
    
    /**
     * Teste para verificar o salvamento de eletrônicos no controle de estoque.
     * Verifica se um eletrônico é salvo corretamente no estoque e se a lista de eletrônicos é atualizada corretamente.
     */
    
    @Test
    public void testSalvarEletronicos() {
        Eletronico eletronico = new Eletronico("Smartphone", "Apple", "", "", 2, 3, 5, 6, null);
        ControleEstoque controle = new ControleEstoque();
        
        boolean resultado = controle.salvareletronicos(eletronico);
        
        assertTrue(resultado);
        assertEquals(1, controle.retornaeletronicos().size());
        assertTrue(controle.retornaeletronicos().contains(eletronico));
    }

}
